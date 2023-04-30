import { useState } from "react";
import { TicTacToeMove } from "../../types";
import styled from "styled-components";

const StyledSquare = styled.div<{ color: string; yourTurn: boolean }>`
  display: flex;
  height: 50px;
  width: 50px;
  text-align: center;
  border: 1px solid black;
  color: black;
  cursor: ${(p) => (p.yourTurn ? "pointer" : "not-allowed")};
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
        opacity: ${(p) => (p.yourTurn ? 1 : 0)};
        visibility: visible;
      }
    }
  }
`;

const Square = ({
  marker,
  onClick,
  yourTurn,
}: {
  marker: string | null;
  onClick: () => void;
  yourTurn: boolean;
}) => {
  return (
    <StyledSquare
      onClick={onClick}
      color={marker === "x" || !marker ? "red" : "blue"}
      yourTurn={yourTurn}
    >
      <span className={marker ? "" : "free"}>{marker ?? "x"} </span>
    </StyledSquare>
  );
};

const StyledBoard = styled.div`
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: center;
  padding: 5px;
  width: 250px;
  .status {
    display: flex;
    flex-direction: column;
    color: black;
    font-size: 1rem;
  }
`;

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
        <span style={{ color: "white" }}>Waiting for player...</span>
      </div>
    );
  }
  const yourTurn = turn === playerId;

  return (
    <StyledBoard style={{ backgroundColor: "white" }}>
      <button
        onClick={onClose}
        style={{ backgroundColor: "white", color: "black" }}
      >
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
          {[0, 1, 2].map((x) =>
            [0, 1, 2].map((y) => (
              <Square
                key={x.toString() + y.toString()}
                yourTurn={yourTurn}
                marker={getMarker(x, y)}
                onClick={() => submitMove(x, y)}
              />
            ))
          )}
        </div>
      ) : (
        <button style={{ color: "black" }}>Create new game</button>
      )}
      <div className="status">
        <span>{yourTurn ? "Your turn" : "Opponents turn"}</span>
        <span>Game status: {gameStatus}</span>
      </div>
    </StyledBoard>
  );
};
