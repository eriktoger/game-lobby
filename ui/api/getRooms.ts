import { Room } from "components/types";
import { baseUrl } from "./constants";

export default async function getRooms(): Promise<Room[]> {
  try {
    const url = `${baseUrl}/rooms`;
    let result = await fetch(url);
    return result.json();
  } catch (e) {
    console.log(e);
    return Promise.resolve(null);
  }
}
