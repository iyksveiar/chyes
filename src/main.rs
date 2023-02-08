use ncurses::*;

fn main() {
  // Initialize ncurses
  initscr();
  noecho();

  // Get the size of the terminal window
  let mut rows = 0;
  let mut cols = 0;
  getmaxyx(stdscr(), &mut rows, &mut cols);

  // Print "Hello World!" in the center of the screen
  mvprintw(rows / 2, (cols - 11) / 2, "Hello World!");

  // Refresh the screen to make the changes visible
  refresh();

  // Wait for the user to press a key before exiting
  loop {
    let ch = getch();
    if ch == 'q' as i32 {
      break
    }
  }

  // Cleanup ncurses
  endwin();
}
