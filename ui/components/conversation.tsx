import React, { useEffect, useRef } from "react";
import Avatar from "./avatar";
import { DisplayMessage, User } from "./types";

function ConversationItem({
  right,
  content,
  username,
}: {
  right: boolean;
  content: string;
  username: string;
}) {
  if (right) {
    return (
      <div className="w-full flex justify-end">
        <div className="flex gap-3 justify-end">
          <div className="max-w-[65%] bg-violet-500 p-3 text-sm rounded-xl rounded-br-none">
            <p className="text-white">{content}</p>
          </div>
          <div className="mt-auto">
            <Avatar>{username}</Avatar>
          </div>
        </div>
      </div>
    );
  }
  return (
    <div className="flex gap-3 w-full">
      <div className="mt-auto">
        <Avatar color="rgb(245 158 11)">{username}</Avatar>
      </div>
      <div className="max-w-[65%] bg-gray-200 p-3 text-sm rounded-xl rounded-bl-none">
        <p>{content}</p>
      </div>
    </div>
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
    <div className="p-4 space-y-4 overflow-auto h-full" ref={ref}>
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
