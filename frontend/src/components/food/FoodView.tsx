import { ReactElement, useState } from "react";
import { useSelector } from "react-redux";
import { IFoodResponse } from "../../messages/Food";
import { IUserInfo, Role } from "../../model/UserInfo";
import { IUserInfoState } from "../../store/user-info";
import EditFoodForm, { IEditEvents } from "./EditFoodForm";

export default function FoodView(props: {
    food: IFoodResponse,
    onEditEvent: IEditEvents,
    userInfo: IUserInfo,
}): ReactElement {
    let [isEditingMode, setIsEditingMode] = useState<boolean>(false);
    const userInfo = useSelector((state: { userInfo: IUserInfoState }) => state.userInfo.userInfo);

    function editHandler() {
        setIsEditingMode(true);
    }

    const editEventHandler: IEditEvents = {
        onEdited: (id: string) => {
            setIsEditingMode(false);
            props.onEditEvent.onEdited(id);
        },
        onCancelled: (id: string) => {
            setIsEditingMode(false);
            props.onEditEvent.onCancelled(id);
        },
        onDeleted: (id: string) => {
            setIsEditingMode(false);
            props.onEditEvent.onDeleted(id);
        }
    };

    const dt = new Date(Date.parse(props.food.time));

    return (
        <>
            {isEditingMode
                ?
                <>
                    <div key={props.food.id}>
                        <EditFoodForm
                            food={props.food}
                            onEditEvent={editEventHandler}
                            userInfo={props.userInfo}
                        />
                    </div>
                </>
                :
                <>
                    <p><b>{props.food.name}</b></p>
                    <p>{props.food.calories} kcal</p>
                    <p>{dt.toLocaleString()}</p>

                    {userInfo?.role === Role.Admin
                        ?
                        <button
                            className="btn btn-primary"
                            type="button"
                            onClick={editHandler}
                        >
                            Edit
                        </button>
                        :
                        <></>
                    }
                </>
            }
        </>
    );
}
