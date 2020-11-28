import { useState, useEffect } from "react";

export default function Home() {
  const [remaining, setRemaining] = useState(0);
  useEffect(() => {
    const i = setInterval(() => {
      setRemaining(new Date().setHours(21, 0, 0, 0) - Date.now());
    }, 1000);
    return () => clearInterval(i);
  }, []);

  const secs = Math.floor(remaining / 1000);
  const minutes = Math.floor(secs / 60);
  const seconds = Math.floor(secs - minutes * 60);
  return (
    <div
      style={{
        textAlign: "center",
        fontSize: 80,
        color: "white",
        fontFamily: "sans-serif",
      }}
    >
      {remaining < 0
        ? ""
        : `${(minutes + "").padStart(2, "0")}:${(seconds + "").padStart(
            2,
            "0"
          )}`}
    </div>
  );
}
