export type Learning = {
  available_at: string;
  subject_ids: number[];
};

export type Summary = {
  lessons: Learning[];
  next_reviews_at: string;
  reviews: Learning[];
};

// Format an ISO timestamp (with Z) as a short hour string like "6pm" or "11am"
export function formatHour(iso: string): string {
  try {
    const d = new Date(iso);
    if (isNaN(d.getTime())) return "-";
    let hours = d.getHours();
    const ampm = hours >= 12 ? "pm" : "am";
    hours = hours % 12;
    if (hours === 0) hours = 12;
    return `${hours}${ampm}`;
  } catch (e) {
    return "-";
  }
}

export function firstCount(list: Learning[] | undefined): number {
  if (!list || list.length === 0) return 0;
  return list[0].subject_ids?.length ?? 0;
}

export function remainingTotal(list: Learning[] | undefined): number {
  if (!list || list.length <= 1) return 0;
  return list
    .slice(1)
    .reduce((acc, l) => acc + (l.subject_ids?.length ?? 0), 0);
}
