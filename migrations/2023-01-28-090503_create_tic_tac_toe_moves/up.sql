CREATE TABLE tic_tac_toe_moves(
  id TEXT PRIMARY KEY NOT NULL,
  player_id TEXT NOT NULL, 
  game_id TEXT NOT NULL,
  row_number Integer NOT NULL,
  column_number Integer NOT NULL,
  created_at TEXT NOT NULL
) 