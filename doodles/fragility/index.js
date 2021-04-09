import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";

function dtSecs() {
  const dt = new Date("2021-06-21") - Date.now();
  return Math.floor(dt / 1000);
}

function format(dt) {
  const days = dt / (24 * 60 * 60);
  if (days < 0) return;
  if (days > 1) return Math.floor(days) + " days";
  const hours = dt / (60 * 60);
  if (hours > 1) return Math.floor(hours) + " hours";
  const minutes = dt / 60;
  if (minutes > 1) return Math.floor(minutes) + " minutes";
  return dt + " seconds!!!";
}

const Main = () => {
  const [dt, setDt] = useState(dtSecs);
  useEffect(() => {
    setInterval(() => {
      setDt(dtSecs());
    }, 1000);
  }, [setDt]);
  const txt = format(dt);
  const binary = dt.toString(2);
  return (
    <>
      {dt < 0 ? null : <header>{binary}</header>}
      <div className="main">
        {!txt ? null : <span>This NFT will self destruct in {txt}</span>}
      </div>
      {dt < 0 ? null : <footer>{binary}</footer>}
    </>
  );
};

ReactDOM.render(<Main />, document.getElementById("main"));
