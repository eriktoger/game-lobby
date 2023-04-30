import React, { useEffect, useRef } from "react";
import Avatar from "./avatar";
import { DisplayMessage, User } from "./types";
import styled from "styled-components";

const StyledConversationItem = styled.div<{ right: boolean }>`
  display: flex;
  flex-direction: ${(p) => (p.right ? "row-reverse" : "row")};
  gap: 10px;
  margin-bottom: 10px;

  p {
    color: white;
    line-height: 30px;
    text-align: ${(p) => (p.right ? "end" : "start")};
    overflow-wrap: break-word;
    word-wrap: break-word;
    hyphens: auto;
    max-width: 80vw;
    background-color: ${(p) => (p.right ? "#680000" : "#010155")};
    padding: 5px;
    border-radius: 10px;
  }
`;

function ConversationItem({
  right,
  content,
  username,
}: {
  right: boolean;
  content: string;
  username: string;
}) {
  return (
    <StyledConversationItem right={right}>
      <Avatar color={right ? "#a00" : "#0101ae"}>{username}</Avatar>
      <p>{content}</p>
    </StyledConversationItem>
  );
}

export default function Conversation({
  data,
  auth,
}: {
  data: DisplayMessage[];
  auth: User;
}) {
  const ref = useRef<any>(null);
  useEffect(() => {
    ref.current?.scrollTo(0, ref.current.scrollHeight);
  }, [data]);
  return (
    <div ref={ref}>
      {data.map((item, i) => {
        return (
          <ConversationItem
            right={item.username === auth.username}
            content={item.content}
            username={item.username}
            key={i}
          />
        );
      })}
    </div>
  );
}
