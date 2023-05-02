import { baseUrl } from "./constants";

export default async function createAccount({
  username,
  phone,
}: {
  username: string;
  phone: string;
}) {
  try {
    const url = `${baseUrl}/users/create`;
    let result = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ username, phone }),
    });
    if (result.ok) {
      const user = await result.json();
      return user;
    }
  } catch (e) {
    console.error(e);
  }
}
