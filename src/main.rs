use std::collections::VecDeque;

use grid::Grid;
use yew::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Player {
    fn color(&self) -> &'static str {
        match self {
            Player::Red => "red",
            Player::Green => "green",
            Player::Blue => "blue",
            Player::Yellow => "yellow",
        }
    }

    fn next_player(&self) -> Player {
        match self {
            Player::Red => Player::Green,
            Player::Green => Player::Blue,
            Player::Blue => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
}

#[derive(Clone, Default)]
struct Cell {
    dot_count: u32,
    owner: Option<Player>,
}

impl Cell {
    fn reset(&mut self) {
        self.dot_count = 0;
        self.owner = None;
    }
}

trait Game {
    fn get_adjacent_indices(&self, row: usize, col: usize) -> Vec<(usize, usize)>;
}

impl Game for Grid<Cell> {
    fn get_adjacent_indices(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();
        let max_row = self.rows();
        let max_col = self.cols();

        // Check if the adjacent cells are within the grid bounds
        if row > 0 {
            adjacent.push((row - 1, col)); // Up
        }
        if row + 1 < max_row {
            adjacent.push((row + 1, col)); // Down
        }
        if col > 0 {
            adjacent.push((row, col - 1)); // Left
        }
        if col + 1 < max_col {
            adjacent.push((row, col + 1)); // Right
        }

        adjacent
    }
}

impl Model {
    fn increment_count(&mut self, row: usize, col: usize, player: Player) -> bool {
        let cell = self.grid.get(row, col).unwrap();
        if cell.owner.is_none() || cell.owner == Some(player) {
            self.increment_count_inner(row, col, player);
            return true;
        }
        return false;
    }

    fn increment_count_inner(&mut self, row: usize, col: usize, player: Player) {
        let mut queue = VecDeque::new();
        queue.push_back((row, col));
        while let Some((r, c)) = queue.pop_front() {
            let adjacent_indices = self.grid.get_adjacent_indices(r, c);
            let cell = self.grid.get_mut(r, c).unwrap(); // For now
            cell.dot_count += 1;
            cell.owner = Some(player);
            // Full! Start a chain reaction
            if cell.dot_count as usize == adjacent_indices.len() {
                cell.reset();
                for (ar, ac) in adjacent_indices {
                    queue.push_back((ar, ac));
                }
            }
        }
    }
}

struct Model {
    grid: Grid<Cell>,
    current_player: Player,
    last_edited: Option<(usize, usize)>,
}

enum Msg {
    AddDot(usize, usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            grid: Grid::new(10, 10),
            current_player: Player::Red,
            last_edited: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddDot(row, col) => {
                if self.increment_count(row, col, self.current_player) {
                    self.last_edited = Some((row, col));
                    self.current_player = self.current_player.next_player();
                }
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = self.grid.iter_rows().enumerate().map(|(row_idx, row)| {
            let cells = row.enumerate().map(|(col_idx, cell)| {
                let onclick = ctx.link().callback(move |_| Msg::AddDot(row_idx, col_idx));
                let color = cell.owner.map_or("white", |p| p.color());
                html! {
                    <td {onclick} style={format!("background-color: {}; width: 50px; height: 50px; text-align: center; border: 1px solid #ddd; cursor: pointer;", color)}>
                        { cell.dot_count }
                    </td>
                }
            });
            html! { <tr>{ for cells }</tr> }
        });

        html! {
            <div>
                <h1>{ "Chain Reaction Game" }</h1>
                <h2 id="current-player">{ format!("Current Player: {}", self.current_player.color()) }</h2>
                <table id="game-grid">
                    <tbody>
                        { for grid }
                    </tbody>
                </table>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
