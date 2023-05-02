import { baseUrl } from "./constants";

export default async function getRoom(room_id: string) {
  const url = `${baseUrl}/rooms/${room_id}/data`;
  try {
    const resp = fetch(url).then((res) => res.json());
    return resp;
  } catch (e) {
    console.log(e);
  }
}
