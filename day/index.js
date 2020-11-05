import * as Day001 from "./001";
import * as Day002 from "./002";
import * as Day003 from "./003";
import * as Day004 from "./004";
import * as Day005 from "./005";
import * as Day006 from "./006";
import * as Day007 from "./007";

const days = [Day001, Day002, Day003, Day004, Day005, Day006, Day007];

import * as Day099 from "./ideas/099";
import * as Day098 from "./ideas/098";
import * as Day097 from "./ideas/097";
if (process.env.NODE_ENV === "development") {
  days.unshift(Day097);
  days.unshift(Day098);
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
