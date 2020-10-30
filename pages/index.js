import { Page } from "../components/Page";
import { getToday } from "../day";

export default function Home() {
  return <Page Day={getToday()} />;
}
