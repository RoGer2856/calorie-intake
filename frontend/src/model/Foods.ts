import { IFoodResponse } from "../messages/Food";

export interface DayFoods {
  dateOfMonth: number,
  dayOfTheWeek: number,
  foods: IFoodResponse[],
}

export function createDayFoods(dateOfMonth: number, dayOfTheWeek: number): DayFoods {
  return {
    dateOfMonth,
    dayOfTheWeek,
    foods: [],
  };
}

export function addFoodToDay(foods: DayFoods, food: IFoodResponse) {
  foods.foods.push(food);
  foods.foods.sort((a: IFoodResponse, b: IFoodResponse) => {
    const aDt = Date.parse(a.time);
    const bDt = Date.parse(b.time);
    return aDt > bDt ? 1 : -1;
  });
}

export interface MonthFoods {
  month: number;
  days: { [day: number]: DayFoods };
}

export function createMonthFoods(month: number): MonthFoods {
  return {
    month,
    days: {},
  };
}

export function addFoodToMonth(foods: MonthFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const dateOfMonth = dt.getDate();
  const dayOfTheWeek = dt.getDay();

  if (!(dateOfMonth in foods.days)) {
    foods.days[dateOfMonth] = createDayFoods(dateOfMonth, dayOfTheWeek);
  }

  addFoodToDay(foods.days[dateOfMonth], food);
}

export function monthToSortedArray(foods: MonthFoods): DayFoods[] {
  let ret = Object.values(foods.days);
  ret.sort((a: DayFoods, b: DayFoods) => {
    return a.dateOfMonth < b.dateOfMonth ? 1 : -1;
  });
  return ret;
}

export interface YearFoods {
  year: number;
  months: { [month: number]: MonthFoods };
}

export function createYearFoods(year: number): YearFoods {
  return {
    year,
    months: {},
  };
}

export function addFoodToYear(foods: YearFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const month = dt.getMonth();

  if (!(month in foods.months)) {
    foods.months[month] = createMonthFoods(month);
  }

  addFoodToMonth(foods.months[month], food);
}

export function yearToSortedArray(foods: YearFoods): MonthFoods[] {
  let ret = Object.values(foods.months);
  ret.sort((a: MonthFoods, b: MonthFoods) => {
    return a.month < b.month ? 1 : -1;
  });
  return ret;
}

export interface AllFoods {
  years: YearFoods[];
}

export function createAllFoods(): AllFoods {
  return {
    years: [],
  };
}

export function addFoodToAll(foods: AllFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const year = dt.getFullYear();

  if (!(year in foods.years)) {
    foods.years[year] = createYearFoods(year);
  }

  addFoodToYear(foods.years[year], food);
}

export function allFoodsToSortedArray(foods: AllFoods): YearFoods[] {
  let ret = Object.values(foods.years);
  ret.sort((a: YearFoods, b: YearFoods) => {
    return a.year < b.year ? 1 : -1;
  });
  return ret;
}
