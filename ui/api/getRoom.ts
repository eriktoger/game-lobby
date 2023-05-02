import { baseUrl } from "./constants";

export default async function getRoom(room_id: string) {
  const url = `${baseUrl}/rooms/${room_id}/data`;
  try {
    const response = await fetch(url);
    const result = await response.json();
    return result;
  } catch (e) {
    console.log(e);
  }
}
