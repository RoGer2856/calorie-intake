import { IUserInfo, Role } from "../model/UserInfo";

export interface IErrorMessage {
    reason: string,
}

export interface IGetUserInfoResponse {
    username: string,
    role: string,
    max_calories_per_day: number,
}

export function userInfoResponseToUserInfo(userInfoResponse: IGetUserInfoResponse): IUserInfo {
    let role = Role.RegularUser;
    switch (userInfoResponse.role) {
      case "regular_user": {
        role = Role.RegularUser;
        break;
      }
      case "admin": {
        role = Role.Admin;
        break;
      }
    }

    let ret: IUserInfo = {
      username: userInfoResponse.username,
      role,
      maxCaloriesPerDay: userInfoResponse.max_calories_per_day,
    };

    return ret;
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

export interface IUpdateFoodRequest {
    name: string | null,
    calories: number | null,
    time: string | null,
}

export interface IFoodRequest {
    name: string,
    calories: number,
    time: string,
}

export interface IGetFoodListResponse {
    foods: IFoodResponse[],
}

export interface IGetFoodReportResponse {
    food_entries_added_last_week: number,
    food_entries_added_week_before_last_week: number,
    average_calories_consumed_last_week: number,
}

export interface IGetUserListResponse {
    users: IGetUserInfoResponse[],
}
