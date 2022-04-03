export interface IFoodResponse {
    id: String,
    name: String,
    calories: number,
    time: String,
}

export interface IFoodRequest {
    name: String,
    calories: number,
    time: String,
}

export interface IGetFoodListResponse {
    foods: IFoodResponse[],
}
