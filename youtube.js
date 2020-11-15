// @flow

const firstTimestamp = 1605362400000;
const episodes = ["7_xNlfX2q9c", "6Xuls7CjQuM"];

const nextYoutubeId = episodes[episodes.length - 1];
const nextYoutubeTimestamp =
  firstTimestamp + (episodes.length - 1) * 7 * 24 * 60 * 60 * 1000;

// TODO how to fetch this?
export function useNextYoutube() {
  return { id: nextYoutubeId, timestamp: nextYoutubeTimestamp };
}
