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

<img width="1326" height="872" alt="image-20260331181015689" src="https://github.com/user-attachments/assets/285f37f1-ebd1-4a7a-b825-6642dec4f364" />


The main interface, at the top, mainly contains several option menus where you can make choices.









### Section 02 Configuration Selection

<img width="1310" height="862" alt="image-20260331181135850" src="https://github.com/user-attachments/assets/2b2c807f-eb2b-42b8-8114-22274c437dca" />



You can choose the path to Rust and the path to the godot file.

Where:

1. The Rust path refers to the folder where Cargo is located.

2. The godot path refers to the location of the godot .exe file.

Of course, you can also set the language. The option to set the software language is clicked below.

<img width="1309" height="864" alt="image-20260331181355306" src="https://github.com/user-attachments/assets/95f006af-56d7-420d-8812-8ec167019b71" />













### Section 3 Project Building

In the project interface, you can begin building your project.


<img width="1304" height="859" alt="image-20260331181508488" src="https://github.com/user-attachments/assets/e6860c28-5a09-4313-875b-04d5c18f432d" />



Follow the button sequence to complete the project build. Wait until the progress bar reaches 100% to complete the entire project build.





The build process is shown below:

<img width="1308" height="853" alt="image-20260331181707163" src="https://github.com/user-attachments/assets/043a58bb-3dd7-4930-bb1e-a7550e836b97" />


<img width="1301" height="859" alt="image-20260331181754200" src="https://github.com/user-attachments/assets/c8c37427-1b43-4702-92fc-4b13660ae7ab" />






When all operations are complete, the system will display "All operations are complete", indicating that the build process is now finished.











### Section 4 About Software

If you encounter any issues while using the software, you can refer to the relevant documentation in the "About the Software" section.


<img width="1302" height="861" alt="image-20260331181959363" src="https://github.com/user-attachments/assets/5887aba2-e2e7-41e6-ae7b-44d47f156cec" />



Instructions for Use

<img width="1303" height="861" alt="image-20260331182024547" src="https://github.com/user-attachments/assets/6488cf81-b53a-4bff-9000-8fab523b1a3a" />












## Chapter 3 Project Display

### Section 01 Project Documents

<img width="1352" height="693" alt="image" src="https://github.com/user-attachments/assets/6fa20d90-bf05-4a79-a444-0c295d21f945" />











### Section 02 godot file

<img width="1562" height="965" alt="image" src="https://github.com/user-attachments/assets/cec9c576-7a7e-47e5-9fdb-3c3c608631f6" />









### Section 3: Godot Execution Results

<img width="1853" height="1001" alt="image" src="https://github.com/user-attachments/assets/55fa0ff3-31c1-4d2b-8bec-0cc4522aad73" />











