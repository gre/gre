import * as Day001 from "./001";
import * as Day002 from "./002";
import * as Day003 from "./003";
import * as Day004 from "./004";
import * as Day005 from "./005";

const days = [Day001, Day002, Day003, Day004, Day005];

import * as cosmos from "./ideas/cosmos";
import * as Day099 from "./ideas/099";
if (process.env.NODE_ENV === "development") {
  days.unshift(cosmos);
  days.unshift(Day099);
}

export function getDays() {
  return days.map((d) => d.n);
}

export function getToday() {
  return days[days.length - 1];
}

export function findDay(n) {
  return days.find((d) => d.n === n);
}
