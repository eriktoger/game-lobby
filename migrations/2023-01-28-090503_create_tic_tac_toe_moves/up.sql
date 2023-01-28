CREATE TABLE tic_tac_toe_moves(
  id TEXT PRIMARY KEY NOT NULL,
  game_id TEXT NOT NULL,
  row_number TEXT NOT NULL,
  col_number TEXT NOT NULL,
  created_at TEXT NOT NULL
)