import { baseUrl } from "./constants";

export default async function getRooms() {
  try {
    const url = `${baseUrl}/rooms`;
    let result = await fetch(url);
    return result.json();
  } catch (e) {
    console.log(e);
    return Promise.resolve(null);
  }
}
