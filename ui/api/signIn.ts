import { baseUrl } from "./constants";

export default async function signIn({ phone }: { phone: string }) {
  try {
    const url = `${baseUrl}/users/phone/${phone}`;
    let response = await fetch(url);
    let result = await response.json();
    return result;
  } catch (e) {
    console.error(e);
    return;
  }
}
