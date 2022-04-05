import { IFoodResponse } from "../messages/Food";

export interface IDayFoods {
  dateOfMonth: number,
  dayOfTheWeek: number,
  foods: IFoodResponse[],
}

export function createDayFoods(dateOfMonth: number, dayOfTheWeek: number): IDayFoods {
  return {
    dateOfMonth,
    dayOfTheWeek,
    foods: [],
  };
}

export function addFoodToDay(foods: IDayFoods, food: IFoodResponse) {
  foods.foods.push(food);
  foods.foods.sort((a: IFoodResponse, b: IFoodResponse) => {
    const aDt = Date.parse(a.time);
    const bDt = Date.parse(b.time);
    return aDt > bDt ? 1 : -1;
  });
}

export interface IMonthFoods {
  month: number;
  days: { [day: number]: IDayFoods };
}

export function createMonthFoods(month: number): IMonthFoods {
  return {
    month,
    days: {},
  };
}

export function addFoodToMonth(foods: IMonthFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const dateOfMonth = dt.getDate();
  const dayOfTheWeek = dt.getDay();

  if (!(dateOfMonth in foods.days)) {
    foods.days[dateOfMonth] = createDayFoods(dateOfMonth, dayOfTheWeek);
  }

  addFoodToDay(foods.days[dateOfMonth], food);
}

export function monthToSortedArray(foods: IMonthFoods): IDayFoods[] {
  let ret = Object.values(foods.days);
  ret.sort((a: IDayFoods, b: IDayFoods) => {
    return a.dateOfMonth < b.dateOfMonth ? 1 : -1;
  });
  return ret;
}

export interface IYearFoods {
  year: number;
  months: { [month: number]: IMonthFoods };
}

export function createYearFoods(year: number): IYearFoods {
  return {
    year,
    months: {},
  };
}

export function addFoodToYear(foods: IYearFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const month = dt.getMonth();

  if (!(month in foods.months)) {
    foods.months[month] = createMonthFoods(month);
  }

  addFoodToMonth(foods.months[month], food);
}

export function yearToSortedArray(foods: IYearFoods): IMonthFoods[] {
  let ret = Object.values(foods.months);
  ret.sort((a: IMonthFoods, b: IMonthFoods) => {
    return a.month < b.month ? 1 : -1;
  });
  return ret;
}

export interface IAllFoods {
  years: IYearFoods[];
}

export function createAllFoods(foodList: IFoodResponse[] | null): IAllFoods {
  let allFoods = {
    years: [],
  };

  if (foodList !== null) {
    for (const food of foodList) {
      addFoodToAll(allFoods, food);
    }
  }
  
  return allFoods;
}

export function addFoodToAll(foods: IAllFoods, food: IFoodResponse) {
  const dt = new Date(Date.parse(food.time));
  const year = dt.getFullYear();

  if (!(year in foods.years)) {
    foods.years[year] = createYearFoods(year);
  }

  addFoodToYear(foods.years[year], food);
}

export function allFoodsToSortedArray(foods: IAllFoods): IYearFoods[] {
  let ret = Object.values(foods.years);
  ret.sort((a: IYearFoods, b: IYearFoods) => {
    return a.year < b.year ? 1 : -1;
  });
  return ret;
}
