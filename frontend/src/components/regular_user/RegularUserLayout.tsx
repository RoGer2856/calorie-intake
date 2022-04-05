import { ReactElement, ReactNode } from "react";
import { NavLink } from "react-router-dom";
import { IUserInfo } from "../../model/UserInfo";

export default function RegularUserLayout(props: {
    userInfo: IUserInfo,
    children: ReactNode
}): ReactElement {
    const navLinkClassName = (v: { isActive: boolean }) => {
        if (v.isActive) {
            return "nav-item active";
        } else {
            return "nav-item";
        }
    };

    return (
        <>
        <nav className="navbar navbar-expand-lg navbar-light bg-light">
            <div className="container-fluid">
                <a className="navbar-brand" href="#">Diet</a>
                <button className="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarText" aria-controls="navbarText" aria-expanded="false" aria-label="Toggle navigation">
                    <span className="navbar-toggler-icon"></span>
                </button>
                <div className="collapse navbar-collapse" id="navbarText">
                    <ul className="navbar-nav me-auto mb-2 mb-lg-0">
                        <li className="nav-item">
                            <NavLink
                                className={navLinkClassName}
                                to="/my-consumption"
                            >
                                My consumption
                            </NavLink>
                        </li>
                    </ul>
                    <span className="navbar-text">
                        Hello <b>{props.userInfo.username}</b>!
                    </span>
                </div>
            </div>
        </nav>
        <div className="m-3">
            {props.children}
        </div>
        </>
    );
}
