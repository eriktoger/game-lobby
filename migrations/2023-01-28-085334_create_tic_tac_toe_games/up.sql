CREATE TABLE tic_tac_toe_games(
  id TEXT PRIMARY KEY NOT NULL,
  player_1 TEXT NOT NULL,
  player_2 TEXT,
  turn TEXT,
  game_status TEXT NOT NULL,
  created_at TEXT NOT NULL
)