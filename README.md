# Simula 25

A modern simulation programming language designed for the 21st century, combining the power of discrete event simulation with modern language features.

## Historical Context

Simula 25 builds upon the rich legacy of simulation programming languages, tracing its roots back to Simula 67, created by Ole-Johan Dahl and Kristen Nygaard at the Norwegian Computing Center in the 1960s. Their groundbreaking work not only pioneered simulation programming but also laid the foundation for object-oriented programming as we know it today.

### The Birth of Simula

The journey began in 1961 when Nygaard started working on simulation languages at the Norwegian Defense Research Establishment. By 1962, he had joined forces with Dahl, and together they created Simula I, the first language specifically designed for simulation. Their work culminated in Simula 67, which introduced revolutionary concepts that would shape computing for decades to come.

As Nygaard later reflected:
> "We were not trying to invent a new programming paradigm. We were trying to solve a problem: how to describe complex systems in a way that was natural to the problem domain."

Dahl added:
> "The key insight was that simulation models should be described in terms of the system being simulated, not in terms of the computer."

### Key Historical Milestones

- **1961**: Nygaard begins work on simulation languages
- **1962**: Dahl joins the project, leading to Simula I
- **1967**: Simula 67 is released, introducing classes and objects
- **1970**: First international Simula User's Group meeting
- **1973**: Simula 67 becomes an international standard
- **2001**: Dahl and Nygaard receive the ACM A.M. Turing Award
- **2002**: The term "object-oriented programming" is officially recognized in computing literature

### Influence on Programming Languages

Simula's influence extended far beyond simulation, inspiring numerous programming languages:

- **Smalltalk**: Alan Kay's vision of object-oriented programming was directly inspired by Simula
- **C++**: Bjarne Stroustrup based C++'s class system on Simula's concepts
- **Java**: Inherited many object-oriented concepts from Simula through C++
- **Python**: Adopted Simula's approach to object-oriented design
- **Ruby**: Matsumoto explicitly cited Simula as an influence

### Simulation Language Evolution

Following Simula, several specialized simulation languages emerged:

- **GPSS** (1961): General Purpose Simulation System
- **SIMSCRIPT** (1963): First simulation language with English-like syntax
- **SIMULA** (1967): First object-oriented simulation language
- **SLAM** (1979): Simulation Language for Alternative Modeling
- **Arena** (1993): Modern discrete event simulation software
- **AnyLogic** (2000): Multi-method simulation platform

Dahl and Nygaard's vision was to create a language that could naturally express simulation models, leading to the development of concepts like classes, objects, and inheritance. Their work influenced generations of programming languages and continues to shape how we think about software development.

Simula 25 honors this legacy by modernizing these concepts while maintaining the core principles that made Simula revolutionary:
- Process-oriented simulation
- Object-oriented design
- Discrete event modeling
- Clear separation of model and simulation logic

## Overview

Simula 25 is a new programming language that brings simulation capabilities to the modern era. It combines the best practices from simulation languages like Simula 67 with modern language features, making it easier than ever to create complex simulations.

## Project Structure

- `simula-compiler/` - The Rust-based compiler and language server implementation
- `vs-code-ex/` - VS Code extension providing language support
- `spec.md` - Language specification and design documents
- `compiler.md` - Compiler implementation details
- `pitch.md` - Project pitch and motivation

## Features

### Language Features
- Modern syntax with strong type system
- Built-in simulation primitives
- Concurrency and async/await support
- Pattern matching and algebraic data types
- Memory safety without garbage collection

### Simulation Capabilities
- Discrete event simulation
- Process-oriented simulation
- Statistical distributions
- Resource management
- Event scheduling
- Data collection and analysis

### Development Tools
- VS Code extension with:
  - Syntax highlighting
  - IntelliSense
  - Type checking
  - Simulation preview
  - Language Server Protocol (LSP) support

## Getting Started

### Prerequisites
- Rust toolchain (for compiler development)
- Node.js and npm (for VS Code extension)
- Visual Studio Code (for development)

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/simula-25.git
   cd simula-25
   ```

2. Build the compiler:
   ```bash
   cd simula-compiler
   cargo build
   ```

3. Build the VS Code extension:
   ```bash
   cd vs-code-ex
   npm install
   npm run compile
   ```

## Contributing

We welcome contributions!

## License

This project is licensed under the MIT License

## Acknowledgments

- Inspired by the pioneering work of Ole-Johan Dahl and Kristen Nygaard
- Built with Rust and TypeScript
- Thanks to all contributors and the open-source community
- Special thanks to the Norwegian Computing Center for preserving Simula's legacy