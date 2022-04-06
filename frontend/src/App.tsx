import { useCallback, useEffect, useState } from 'react';
import { useDispatch } from 'react-redux';
import AdminApp from './components/admin/AdminApp';
import Loading from './components/Loading';
import RegularUserApp from './components/regular_user/RegularUserApp';
import UseApiView from './components/UseApiView';
import useApi from './hooks/use-api';
import { IUserInfo, Role } from './model/UserInfo';
import { userInfoActions } from './store/user-info';

function App() {
  const [apiFeedback, api] = useApi();

  const dispatch = useDispatch();

  const [userInfo, setUserInfo] = useState<IUserInfo | null>(null);

  let fetchUserInfo = useCallback(async function () {
    let response = await api.getUserInfo();
    if (response !== null) {
      setUserInfo(response as IUserInfo);
      dispatch(userInfoActions.setUserinfo(response as IUserInfo));
    }
  }, []);

  useEffect(() => {
    fetchUserInfo();
  }, [fetchUserInfo])

  return (
    <>
      <UseApiView apiFeedback={apiFeedback}>
        <>
          {userInfo !== null
            ?
            <>
              {userInfo!.role === Role.RegularUser
                ?
                <RegularUserApp userInfo={userInfo!} />
                :
                <AdminApp userInfo={userInfo!} />
              }
            </>
            :
            <Loading />
          }
        </>
      </UseApiView>
    </>
  );
}

export default App;
