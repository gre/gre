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
import * as Day025 from "./025";
import * as Day026 from "./026";
import * as Day027 from "./027";
import * as Day028 from "./028";
import * as Day029 from "./029";
import * as Day030 from "./030";
import * as Day031 from "./031";
import * as Day032 from "./032";
import * as Day033 from "./033";
import * as Day034 from "./034";
import * as Day035 from "./035";
import * as Day036 from "./036";
import * as Day037 from "./037";
import * as Day038 from "./038";
import * as Day039 from "./039";
import * as Day040 from "./040";
import * as Day041 from "./041";
import * as Day042 from "./042";
import * as Day043 from "./043";
import * as Day044 from "./044";
import * as Day045 from "./045";
import * as Day046 from "./046";
import * as Day047 from "./047";

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
  Day025,
  Day026,
  Day027,
  Day028,
  Day029,
  Day030,
  Day031,
  Day032,
  Day033,
  Day034,
  Day035,
  Day036,
  Day037,
  Day038,
  Day039,
  Day040,
  Day041,
  Day042,
  Day043,
  Day044,
  Day045,
  Day046,
  Day047,
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
