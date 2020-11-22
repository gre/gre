import * as Day001 from "./001";
import * as Day002 from "./002";
import * as Day003 from "./003";
import * as Day004 from "./004";
import * as Day005 from "./005";
import * as Day006 from "./006";
import * as Day007 from "./007";
import * as Day008 from "./008";
import * as Day009 from "./009";
import * as Day010 from "./010";
import * as Day011 from "./011";
import * as Day012 from "./012";
import * as Day013 from "./013";
import * as Day014 from "./014";
import * as Day015 from "./015";
import * as Day016 from "./016";
import * as Day017 from "./017";
import * as Day018 from "./018";
import * as Day019 from "./019";
import * as Day020 from "./020";
import * as Day021 from "./021";
import * as Day022 from "./022";
import * as Day023 from "./023";
import * as Day024 from "./024";

const days = [
  Day001,
  Day002,
  Day003,
  Day004,
  Day005,
  Day006,
  Day007,
  Day008,
  Day009,
  Day010,
  Day011,
  Day012,
  Day013,
  Day014,
  Day015,
  Day016,
  Day017,
  Day018,
  Day019,
  Day020,
  Day021,
  Day022,
  Day023,
  Day024,
];

import * as Day099 from "./ideas/099";
import * as Day097 from "./ideas/097";
if (process.env.NODE_ENV === "development") {
  days.unshift(Day097);
  days.unshift(Day099);
}

export function getDays() {
  return days;
}

export function getToday() {
  return days[days.length - 1];
}

export function findDay(n) {
  return days.find((d) => d.n === n);
}
