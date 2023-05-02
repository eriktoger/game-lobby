import { baseUrl } from "./constants";

export default async function connectSessionId(
  user_id: string,
  session_id: string
) {
  try {
    fetch(`${baseUrl}/users/${user_id}/session/${session_id}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    });
  } catch (error) {
    console.error(error);
  }
}
