# rust-godot-startup













## Chapter 1 Business Pain Points

### Section 01 Problems Encountered

As a novice programmer, during my first experience using Rust and Godot together for development, I encountered the following problems. I spent a lot of time resolving a series of strange errors caused by environment setup during the construction of my first project. As a beginner, I shouldn't have spent so much time on environment issues.

I believe some developers, when first using this method, can create a blank Rust-Godot Git repository, pull it, build, and run it.

However, I understand that this approach may have some shortcomings.

1. It's not suitable for most novice developers.

2. In China, due to network issues, it's impossible to access repositories like `GitHub` normally.

Therefore, I spent about 10 days developing a tool for one-click project building, integrating the Rust and Godot components, from commands to the entire build process.







### Section 2 Problem Solving

A unified Rust godot project build tool has been compiled.

It can build a fast and runnable Rust godot project with a single click through its own deployment operations.

You only need to make a few simple configurations.

For example:

1. Cargo path

2. godot path

3. Workspace

4. Rust root directory name

5. gdext name

6. Whether to create a demo project

After completing the above configurations, you can build a Rust godot project with a single click.

The entire process takes only 2 minutes.









### Section 3 Software Description

1. This software uses the Windows operating system. You can compile the code to generate executable files for other operating systems.

2. This software supports 9 languages, including (English, Simplified Chinese, Traditional Chinese, Japanese, Korean, German, French, Spanish, and Italian).

3. The configuration file only needs to be set once; subsequent settings are cached locally.















## Chapter Two: Interface Introduction

### Section 01: Main Interface

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/b6de0058-5217-48eb-a383-9ef39b8f488b" />



The main interface, at the top, mainly contains several option menus where you can make choices.









### Section 02 Configuration Selection

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/cf612b11-e423-44e0-8bd0-ed1d4f0d02be" />




You can choose the path to Rust and the path to the godot file.

Where:

1. The Rust path refers to the folder where Cargo is located.

2. The godot path refers to the location of the godot .exe file.

Of course, you can also set the language. The option to set the software language is clicked below.

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/021e0b0b-2e1e-4810-8b84-f9f4f614467b" />














### Section 3 Project Building

In the project interface, you can begin building your project.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/e44c68f9-d921-46eb-8b4c-d82def50555c" />




Follow the button sequence to complete the project build. Wait until the progress bar reaches 100% to complete the entire project build.





The build process is shown below:

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/bedbebcc-0aca-4aac-96eb-cff47c98012d" />



<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/f68fb5cb-0072-48fd-ab79-49f43e0d6413" />







When all operations are complete, the system will display "All operations are complete", indicating that the build process is now finished.











### Section 4 About Software

If you encounter any issues while using the software, you can refer to the relevant documentation in the "About the Software" section.


<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/d0c7fbda-1760-4685-a4b8-3773d73ba19c" />




Instructions for Use

<img width="1282" height="838" alt="image" src="https://github.com/user-attachments/assets/c720b0f3-a50b-4d62-8bd1-f5713079d985" />













## Chapter 3 Project Display

### Section 01 Project Documents

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />











### Section 02 godot file

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />









### Section 3: Godot Execution Results

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











