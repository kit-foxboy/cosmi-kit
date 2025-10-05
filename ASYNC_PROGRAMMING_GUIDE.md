# 🚀 Async Programming in COSMIC Apps - Quick Guide

## 🎯 The Pattern: Task::perform

Fundamentally, this isn't so different from JavaScript async/await. COSMIC apps use `cosmic::Task::perform` to run async code. Here's the fundamental pattern:

```rust
cosmic::Task::perform(
    async move {
        // 1. This block runs in a BACKGROUND THREAD
        // 2. Can do slow operations (database, network, file I/O)
        // 3. Must return a value that can be sent back to UI
        
        let result = do_async_work().await?;
        Ok(result)
    },
    |result| {
        // 4. This closure runs in the UI THREAD
        // 5. Converts the result into a Message
        // 6. Message goes to update() method
        
        cosmic::Action::App(Message::OperationComplete(result))
    }
)
```

## 📋 Step-by-Step: Database Operations

### **Step 1: Define Messages**
These messages represent user actions and async results. Everything in a cosmic app is event-driven via messages. It's all about the Model-View-Update (MVU) pattern where messages trigger state changes and that automatically updates the UI.

```rust
#[derive(Debug, Clone)]
pub enum Message {
    // Request messages (user triggers these)
    LoadData,
    CreateProject(String, Option<String>),
    DeleteProject(i64),
    
    // Response messages (async tasks return these)
    ProjectsLoaded(Result<Vec<Project>, String>),
    ProjectCreated(Result<Project, String>),
    ProjectDeleted(Result<(), String>),
}
```

**Key insight**: Two-phase messaging
- **Request**: User action triggers async work
- **Response**: Async work completes and returns result
Think of it like promises in JavaScript: you initiate an action, and later handle the result when it's ready.

### **Step 2: Handle Request in update()**

```rust
pub fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
    match message {
        Message::LoadData => {
            self.is_loading = true;  // Update UI state
            Self::load_projects_task()  // Return the async task
        }
        
        Message::ProjectsLoaded(result) => {
            self.is_loading = false;
            match result {
                Ok(projects) => self.projects = projects,
                Err(e) => self.error = Some(e),
            }
            cosmic::Task::none()  // No further async work
        }
    }
}
```

### **Step 3: Create Task Helper Method**

```rust
fn load_projects_task() -> cosmic::Task<cosmic::Action<Message>> {
    cosmic::Task::perform(
        async move {
            // Background thread work
            let db = SqliteDatabase::new().await
                .map_err(|e| e.to_string())?;  // Convert errors to String
                
            db.get_all_projects().await
                .map_err(|e| e.to_string())
        },
        |result| {
            // Convert Result to Message
            cosmic::Action::App(Message::ProjectsLoaded(result))
        }
    )
}
```

## 🔄 Complete Flow Example

```
User clicks "Load Projects" button
    ↓
Button emits Message::LoadData
    ↓
update() receives Message::LoadData
    ↓
update() sets is_loading = true
    ↓
update() returns load_projects_task()
    ↓
Task spawns async work in background
    ↓
async block connects to database
    ↓
async block queries projects
    ↓
async block returns Result<Vec<Project>, Error>
    ↓
Closure converts to Message::ProjectsLoaded(result)
    ↓
update() receives Message::ProjectsLoaded
    ↓
update() sets self.projects = result
    ↓
update() sets is_loading = false
    ↓
view() re-renders with new data
```

## ⚠️ Common Pitfalls

### **1. Forgetting .await**
Unlike JavaScript where you put await out in front of the function call, in Rust you must use `.await` on the Future inside the async block. Also remember the ? operator for error handling.:
```rust
// ❌ Wrong - returns a Future, doesn't execute
let db = SqliteDatabase::new();

// ✅ Correct - waits for completion
let db = SqliteDatabase::new().await?;
```

### **2. Using &self in async block**
```rust
// ❌ Wrong - can't borrow self across async boundary
async move {
    self.database.query()  // self not available!
}

// ✅ Correct - clone or copy needed data
let name = self.project_name.clone();
async move {
    create_project(name).await
}
```

### **3. Not handling errors**
This is the worst kind of crash because it happens silently in the background thread and the UI just keeps plugging along. Always convert errors to a type that can be sent back to the UI, typically `String` for simplicity.
```rust
// ❌ Wrong - panic in background thread crashes app silently
async move {
    let db = SqliteDatabase::new().await.unwrap();  // Panic!
}

// ✅ Correct - convert to Result and handle in UI
async move {
    let db = SqliteDatabase::new().await
        .map_err(|e| e.to_string())?;
    Ok(db)
}
```

## 🎯 Pattern Templates

### **Read Operation (Query)**
```rust
// Message
Message::LoadThings,
Message::ThingsLoaded(Result<Vec<Thing>, String>),

// Update handler
Message::LoadThings => Self::load_things_task(),
Message::ThingsLoaded(Ok(things)) => {
    self.things = things;
    cosmic::Task::none()
}

// Task helper
fn load_things_task() -> cosmic::Task<cosmic::Action<Message>> {
    cosmic::Task::perform(
        async move {
            let db = Database::new().await.map_err(|e| e.to_string())?;
            db.get_things().await.map_err(|e| e.to_string())
        },
        |result| cosmic::Action::App(Message::ThingsLoaded(result))
    )
}
```

### **Write Operation (Mutation)**
```rust
// Message
Message::CreateThing(String),
Message::ThingCreated(Result<Thing, String>),

// Update handler
Message::CreateThing(name) => {
    self.is_saving = true;
    Self::create_thing_task(name)
}
Message::ThingCreated(Ok(thing)) => {
    self.is_saving = false;
    self.things.push(thing);
    cosmic::Task::none()
}

// Task helper
fn create_thing_task(name: String) -> cosmic::Task<cosmic::Action<Message>> {
    cosmic::Task::perform(
        async move {
            let mut db = Database::new().await.map_err(|e| e.to_string())?;
            db.create_thing(name).await.map_err(|e| e.to_string())
        },
        |result| cosmic::Action::App(Message::ThingCreated(result))
    )
}
```

## 🚀 Advanced: Chaining Tasks

Sometimes you need to do one async operation, then another:

```rust
Message::CreateAndLoad(name) => {
    cosmic::Task::perform(
        async move {
            // First operation
            let mut db = Database::new().await.map_err(|e| e.to_string())?;
            db.create_thing(name).await.map_err(|e| e.to_string())?;
            
            // Second operation using same db
            db.get_all_things().await.map_err(|e| e.to_string())
        },
        |result| cosmic::Action::App(Message::ThingsLoaded(result))
    )
}
```

## 💡 Key Takeaways

1. **Two-phase messages**: Request + Response (or yip + yap in my case)
2. **Task::perform**: Bridges async code to UI messages
3. **Error handling**: Convert all errors to String for Message passing
4. **State management**: Set loading flags before/after async work
5. **Data flow**: Clone data before moving into async block (Ownership is neat but sure fries my noodle at times)

---

**Remember**: The UI thread and async tasks are separate. Tasks communicate back to UI via messages! 
Also note that background operations are a single thread by default, not one thread per operation so you have to be careful about blocking calls. 🦊✨