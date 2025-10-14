const rtf = new Intl.RelativeTimeFormat("en", { numeric: "auto" });

const units: [Intl.RelativeTimeFormatUnit, number][] = [
  ["year", 365 * 24 * 60 * 60],
  ["month", 30 * 24 * 60 * 60],
  ["day", 24 * 60 * 60],
  ["hour", 60 * 60],
  ["minute", 60],
  ["second", 1],
];

export const formatRelativeTime = (timestamp: string): string => {
  const now = Date.now();
  const date = Number(timestamp) * 1000;

  let diffInSeconds = (date - now) / 1000;
  for (const [unit, secondsInUnit] of units) {
    if (Math.abs(diffInSeconds) >= secondsInUnit || unit === "second") {
      const value = Math.round(diffInSeconds / secondsInUnit);
      return rtf.format(value, unit);
    }
  }

  return "Invalid time";
};
