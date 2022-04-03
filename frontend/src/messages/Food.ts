export interface IErrorMessage {
    reason: string,
}

export interface IAddFoodResponse {
    id: string,
}

export interface IFoodResponse {
    id: string,
    name: string,
    calories: number,
    time: string,
}

export interface IFoodRequest {
    name: string,
    calories: number,
    time: string,
}

export interface IGetFoodListResponse {
    foods: IFoodResponse[],
}
