import { Home as HomeComponent } from "../components/Home";
import { getToday } from "../day";

export default function Home() {
  return <HomeComponent Day={getToday()} />;
}
