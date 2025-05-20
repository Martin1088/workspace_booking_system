export const workingDays = [
  { name: "Mon", num: 1 },
  { name: "Tue", num: 2 },
  { name: "Wed", num: 3 },
  { name: "Thu", num: 4 },
  { name: "Fri", num: 5 },
]
export interface Participant {
  id: number;
  username: string;
}

export interface Room {
  id: number;
  room_name: string;
  description: string;
  capacity: number;
  entry_id: number | null;
  registration_limit: number;
  participants: Participant[];
}

export interface Area {
  id: number;
  area_name: string;
  rooms: Room[];
}

export interface ApiMrbsResponse {
  result: Area[];
}
export type MrbsResponse = Area[];

export interface WeekData {
  date: string;
  weekday: string;
  areas: Area[];
}

export type WeekResponse = WeekData[];

export interface ApiWeekResponse {
  result: WeekData[];
}

export const navAdmin = [
  { label: 'Today', id: 1 },
  { label: 'Weekday', id: 2 },
  { label: 'Office Layout', id: 3 },
  { label: 'Admin Create', id: 4 },
];
export const navUser = [
  { label: 'Today', id: 1 },
  { label: 'Weekday', id: 2 },
  { label: 'Office Layout', id: 3 },
];

export type RangeOfDefaultEntries = {
  start: string,
  end: string
}

