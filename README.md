# GridClicker

Overlay over any other program, draw a grid over it, and click neatly on that grid. Works for Windows only.

## Demo
[Demo Video](https://github.com/codynhanpham/grid_clicker/raw/refs/heads/main/static/demo.mp4)

https://github.com/user-attachments/assets/f998c9d6-269d-4275-bc33-3dab50c5e7a2



## Installation

### Pre-built binaries
Download the latest release from the [Releases](https://github.com/codynhanpham/grid_clicker/releases) page.

### Build from source
1. Install [Rust](https://www.rust-lang.org/tools/install) and [Bun](https://bun.sh/).
2. Clone the repository
```bash
git clone https://github.com/codynhanpham/GridClicker.git
```
3. Install dependencies:
```bash
cd GridClicker
bun install
```
4. Build the project:
```bash
bun tauri build

# or to make into a single .exe file (no installer):
bun tauri build --no-bundle
```
5. The executable will be located in the `src-tauri/target/release/bundle` directory.



## Usage
1. Run the executable. The program will be Always-on-Top and transparent such that you can see the grid over any other program.

> Click-through (making the program not intercept direct mouse clicks and direct them to the underlying program) can be toggled by pressing **`F8`** from anywhere (even when this program is not in focus).

2. Use the Draw Grid tool to define a rectangle. You can customize the grid size (row and column count) and alignment in the Grid Settings menu.

3. Optionally, pick a Home position and set how the homing-clicks should interact with the grid.

4. Set the cursor button (left, right, middle) for the grid and home clicks.

5. Set the click timing (hold time and click intervals).

6. Click the Start button to start the grid clicking.

7. The last configuration will be saved and loaded automatically the next time you run the program.
