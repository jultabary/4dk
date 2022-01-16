package org.tby.fourdk.security.userinfo;

import java.util.Map;

public interface UserInfoAccessor {

    UserInfo getUserInfoFromHttpHeaders(Map<String, String> headers);

    String getUserInfoAccessorName();
}
