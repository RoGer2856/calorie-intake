import { IFoodResponse } from "../messages/Food";

export class DayFoods {
    dateOfMonth: number;
    foods: IFoodResponse[] = [];

    constructor(day: number) {
        this.dateOfMonth = day;
    }

    addFood(food: IFoodResponse) {
        this.foods.push(food);
        this.foods.sort((a: IFoodResponse, b: IFoodResponse) => {
                const aDt = Date.parse(a.time.toString());
                const bDt = Date.parse(b.time.toString());
                return aDt < bDt ? 1 : -1;
        })
    }
}

export class MonthFoods {
    month: number;
    days: { [day: number]: DayFoods } = {};

    constructor(month: number) {
        this.month = month;
    }

    addFood(food: IFoodResponse) {
        const dt = new Date(Date.parse(food.time.toString()));
        const dateOfMonth = dt.getDate();

        if (!(dateOfMonth in this.days)) {
            this.days[dateOfMonth] = new DayFoods(dateOfMonth);
        }

        this.days[dateOfMonth].addFood(food);
    }

    toSortedArray(): DayFoods[] {
        let ret = Object.values(this.days);
        ret.sort((a: DayFoods, b: DayFoods) => {
            return a.dateOfMonth < b.dateOfMonth ? 1 : -1;
        });
        return ret;
    }
}

export class YearFoods {
    year: number;
    months: { [month: number]: MonthFoods } = {};

    constructor(year: number) {
        this.year = year;
    }

    addFood(food: IFoodResponse) {
        const dt = new Date(Date.parse(food.time.toString()));
        const month = dt.getMonth();

        if (!(month in this.months)) {
            this.months[month] = new MonthFoods(month);
        }

        this.months[month].addFood(food);
    }

    toSortedArray(): MonthFoods[] {
        let ret = Object.values(this.months);
        ret.sort((a: MonthFoods, b: MonthFoods) => {
            return a.month < b.month ? 1 : -1;
        });
        return ret;
    }
}

export class AllFoods {
    years: YearFoods[] = [];

    addFood(food: IFoodResponse) {
        const dt = new Date(Date.parse(food.time.toString()));
        const year = dt.getFullYear();

        if (!(year in this.years)) {
            this.years[year] = new YearFoods(year);
        }

        this.years[year].addFood(food);
    }

    toSortedArray(): YearFoods[] {
        let ret = Object.values(this.years);
        ret.sort((a: YearFoods, b: YearFoods) => {
            return a.year < b.year ? 1 : -1;
        });
        return ret;
    }
}
