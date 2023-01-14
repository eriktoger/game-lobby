import { useEffect, useState } from "react";
export default function useLocalStorage(key: any, defaultValue: any) {
  const [storedValue, setStoredValue] = useState(defaultValue);
  console.log(storedValue);
  const setValue = (value: any) => {
    try {
      const valueToStore =
        value instanceof Function ? value(storedValue) : value;
      setStoredValue(valueToStore);
      if (typeof window !== "undefined") {
        window.localStorage.setItem(key, JSON.stringify(valueToStore));
      }
    } catch (error) {}
  };
  useEffect(() => {
    try {
      const item = window.localStorage.getItem(key);
      let data = item ? JSON.parse(item) : defaultValue;
      setStoredValue(data);
    } catch (error) {}
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return [storedValue, setValue];
}
