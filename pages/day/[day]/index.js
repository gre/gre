import { useRouter } from "next/router";
import { Page } from "../../../components/Page";
import { findDay } from "../../../day";

export default function Home() {
  const router = useRouter();
  const { day } = router.query;
  const Day = findDay(parseInt(day, 10));
  if (!Day) return null;
  return <Page Day={Day} />;
}
