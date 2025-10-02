# 🦊 Cosmi-Kit: A head first deep dive into the Cosmic beta ecosystem!

> *Brought to you by Kit Kabbit, blowing things up by pushing random buttons since 1987!*

A hands-on learning project for exploring the **COSMIC™ desktop environment** through building real applications. This is less a demo app and more  a living laboratory for the mad science of mastering libcosmic patterns, Rust desktop development, and having fun while doing it with semi-practical exercises and mini-projects! 🚀

## 🎯 What Makes This Special

This project serves as both a **learning template** and a **practical toolkit** for COSMIC development. You can follow the commits and branches to see how I implemented each feature. As I am still a Rust noob, I'll be focusing on these fundamental concepts:

- **🧠 Model-View-Update Architecture** - Learn reactive programming patterns with aggressive type safety
- **🎨 Type-Safe UI Development** - Rust's safety guarantees for desktop apps
- **🌍 Internationalization** - Fluent-based localization from day one
- **⚡ Modern Async Patterns** - Subscriptions, tasks, and message handling (This one scares me the most XwX)
- **🎮 Interactive Widgets** - Buttons, inputs, and dynamic state management

### Current Features (Learning Modules Completed ✅)

#### 📋 **Exercise 1: Navigation System** *(Completed!)*
- **What you'll learn**: MVU pattern, enum-based routing, widget composition
- **Implementation**: Three-page app with functional navigation
  - 🎭 **OC Generator** - Character creation tools
  - 📊 **Project Manager** - Development workflow tracking  
  - 🎲 **Dice Roller** - Gaming utilities
- **Key concepts mastered**: Type-safe navigation, localization workflow, icon theming

### Coming Up Next 🔮

#### 🎯 **Exercise 2: Interactive Widgets & State**
- Add buttons, inputs, and dynamic content to each page
- Learn state management and event handling
- Build your first interactive COSMIC widget

#### 🗃️ **Exercise 3: Persistent Storage**
- Configuration management with CosmicConfigEntry
- File I/O and data persistence
- Settings panels and user preferences

#### 🌐 **Exercise 4: Async & Networking**
- HTTP requests and API integration
- Background tasks and subscriptions
- Real-time data updates

### Future Adventures As Full Blown Projects 🚀
- Custom widgets and complex layouts
- Multi-window applications
- System integration and DBus
- Plugin architectures
- Cross-platform deployment


## 🛠️ Quick Start

A [justfile](./justfile) is included for the [casey/just][just] command runner:

```bash
# Development workflow
just run              # Build and run the app
just check             # Run clippy linting

# Production builds  
just build-release     # Optimized build
just install           # System installation
just vendor            # Create vendored dependencies
```

## 🎨 Learning Philosophy

This project follows a **"learn by setting everything on fire"** approach with these principles:

- **🔥 Devour Documentation** - Read official docs, then experiment with the refs by your side
- **🔍 Incremental Complexity** - Start simple, add features step by step
- **💡 Concept-First Learning** - Understand *why* before *how* because that's a little involved with Rust lol
- **🎮 An Attempt At Fun Projects** - Build things you actually want to use!
- **📚 Rich Documentation of My Suffering** - Every exercise includes detailed explanations

### What Makes This Silliness Worth Learning?

- **Type Safety** - Rust's compile-time guarantees prevent common UI bugs but the learning curve is steep
- **Modern Patterns** - Reactive programming with functional influences
- **Async Rabbit Holes** - Mastering async in Rust is intimidating but as with most languages, powerful
- **System Integration** - Deep integration with the COSMIC desktop environment (I still love you Hyprland, honest!)
- **Testing Cross-Platform Portability** - Runs on Linux, with future Windows/macOS support

## 📖 Learning Resources

### In This Repo
- [`LEARNING_REFLECTION.md`](./LEARNING_REFLECTION.md) - Detailed exercise breakdowns and concept explanations (templated by ai and loaded with my thoughts, ain't nobody got time for that!)
- [`src/app.rs`](./src/app.rs) - Main application with extensive comments
- [`i18n/`](./i18n/) - Localization examples and patterns

### External Documentation
- [📘 libcosmic Book](https://pop-os.github.io/libcosmic-book/introduction.html) - Official getting started guide
- [📚 libcosmic API Docs](https://pop-os.github.io/libcosmic/cosmic/) - Complete API reference
- [🌟 COSMIC Design Guidelines](https://github.com/pop-os/cosmic-epoch) - UI/UX patterns and principles

## 🌍 Localization

[Fluent][fluent] powers our internationalization system. Translation files live in [`i18n/`](./i18n/).

**Adding new languages:**
1. Copy the [`en/`](./i18n/en) directory 
2. Rename to your [ISO 639-1 language code][iso-codes]
3. Translate the message identifiers in the `.ftl` files
4. Messages can be omitted if no translation is needed

**Pro tip**: The `fl!()` macro provides compile-time validation of translation keys!

## 📦 Packaging & Distribution

For Linux distribution packaging:

```bash
# Create vendored dependencies
just vendor

# Build with vendored sources  
just build-vendored

# Install with custom paths
just rootdir=debian/cosmi-kit prefix=/usr install
```

**Recommended workflow**: Run `just vendor` on the host system before entering the build environment to create a source tarball with vendored dependencies.

## 🔧 Development Environment

### Essential Setup (How I did it at least)
1. **Install [rustup][rustup]** - Rust toolchain manager
2. **Configure [rust-analyzer][rust-analyzer]** - IDE language server  
3. **Optional performance boosters**:
   - **[mold][mold]** - Faster linking (disable LTO in dev)
   - **[sccache][sccache]** - Compilation caching

### My VS Code Integration
This project includes configuration for optimal VS Code development:
- Rust-analyzer settings
- Debug configurations  
- Task definitions for just commands
- Officially recommended extensions:
  - rust-analyzer
  - Better TOML
  - CodeLLDB (for debugging)
  - Just (for justfile support)

## 🎉 Contributing

This is a learning project, but contributions are welcome if you want to document your experiences too:
- **🐛 Exercise improvements** - Better explanations, clearer examples
- **🎯 New learning modules** - Additional COSMIC features to explore  
- **🌍 Translations** - Help make learning accessible worldwide (I barely speak English, help!)
- **📚 Documentation** - Clarifications, typos, better organization (I am notoriously bad with typos)

### All You Need Is:
- Be kind and patient with each other, it's a classroom basically
- Give constructive feedback as often as possible
- Celebrate mistakes as learning opportunities, we stan exploding computers in this house 
- Love. Love. Love is all you need

## 🏷️ Project Status

**Current Version**: Learning Template v1.0  
**COSMIC Compatibility**: Works with cosmic-epoch development branch  
**Self Education Goal**: Beginner to Intermediate Rust Desktop Development

---

## 📝 Links & References

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[mold]: https://github.com/rui314/mold
[sccache]: https://github.com/mozilla/sccache

---

*Happy coding, fellow fuzzbutts! 🦊✨*
