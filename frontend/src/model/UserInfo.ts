export enum Role {
    Admin,
    RegularUser,
}

export interface IUserInfo {
    username: string,
    role: Role,
    maxCaloriesPerDay: number,
}