import { useState } from "react";
import { TicTacToeMove } from "../../types";
import styled from "styled-components";

const StyledSquare = styled.div<{ color: string }>`
  display: flex;
  height: 50px;
  width: 50px;
  text-align: center;
  border: 1px solid black;
  color: black;
  cursor: pointer;
  span {
    display: flex;
    height: 100%;
    width: 100%;
    justify-content: center;
    flex-direction: column;
    color: ${(p) => p.color};

    &.free {
      opacity: 0;
      :hover {
        opacity: 1;
        visibility: visible;
      }
    }
  }
`;

const Square = ({
  marker,
  onClick,
}: {
  marker: string | null;
  onClick: () => void;
}) => {
  return (
    <StyledSquare
      onClick={onClick}
      color={marker === "x" || !marker ? "red" : "blue"}
    >
      <span className={marker ? "" : "free"}>{marker ?? "x"} </span>
    </StyledSquare>
  );
};

export const Board = ({
  onClose,
  submitMove,
  gameId,
  moves,
  playerId,
  gameStatus,
  turn,
}: {
  onClose: () => void;
  submitMove: (row: number, column: number) => void;
  gameId?: string;
  moves: TicTacToeMove[];
  playerId: string;
  gameStatus: string | null;
  turn: string | null;
}) => {
  const [currentGame, setCurrentGame] = useState(gameId);

  // I think I need redux or contexts to handle this, since it needs to get it from the webSockets

  // I want to be able to create a new game
  // load a game (if I have the id)
  // and play moves
  // others should be able to join
  //maybe a crete new game button?
  const createNewGame = () => {
    //fetch new game
  };

  const getMarker = (row: number, col: number) => {
    const move = moves.find(
      (move) => move.row_number === row && move.column_number === col
    );
    if (!move) {
      return null;
    }
    return move.player_id === playerId ? "x" : "o";
  };

  if (turn == null) {
    //here could also be a possiblity to invite players?
    return (
      <div>
        <span>Waiting for player...</span>
      </div>
    );
  }
  const yourTurn = turn === playerId;
  return (
    <div style={{ backgroundColor: "white" }}>
      <button onClick={onClose} style={{ color: "black" }}>
        Close
      </button>
      {currentGame ? (
        <div
          style={{
            width: 150,
            height: 150,
            display: "grid",
            gap: "1px",
            gridTemplateColumns: "repeat(3, 1fr)",
            color: "red",
          }}
        >
          <Square marker={getMarker(0, 0)} onClick={() => submitMove(0, 0)} />
          <Square marker={getMarker(0, 1)} onClick={() => submitMove(0, 1)} />
          <Square marker={getMarker(0, 2)} onClick={() => submitMove(0, 2)} />
          <Square marker={getMarker(1, 0)} onClick={() => submitMove(1, 0)} />
          <Square marker={getMarker(1, 1)} onClick={() => submitMove(1, 1)} />
          <Square marker={getMarker(1, 2)} onClick={() => submitMove(1, 2)} />
          <Square marker={getMarker(2, 0)} onClick={() => submitMove(2, 0)} />
          <Square marker={getMarker(2, 1)} onClick={() => submitMove(2, 1)} />
          <Square marker={getMarker(2, 2)} onClick={() => submitMove(2, 2)} />
        </div>
      ) : (
        <button style={{ color: "black" }}>Create new game</button>
      )}
      <span>Game status: {gameStatus}</span>
    </div>
  );
};
