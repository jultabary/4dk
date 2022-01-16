package org.tby.fourdk.security.sample.spring.infrastructure.http;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.stereotype.Component;
import org.tby.fourdk.security.ForbiddenAccessException;
import org.tby.fourdk.security.Role;
import org.tby.fourdk.security.userinfo.UserId;
import org.tby.fourdk.security.userinfo.UserInfo;
import org.tby.fourdk.security.userinfo.UserInfoAccessor;

import java.util.Base64;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Collectors;

@Component
public class UserInfoAccessorOIDC implements UserInfoAccessor {
    @Override
    public UserInfo getUserInfoFromHttpHeaders(Map<String, String> headers) {
        var userInfoPayload = getUserInfoMap(headers);
        var userId = getUserId(userInfoPayload);
        var roles = getRoleList(userInfoPayload);
        return new UserInfo(userId, roles);
    }

    @Override
    public String getUserInfoAccessorName() {
        return "OIDC_FAKE";
    }

    private UserId getUserId(Map<String, Object> userInfoPayload) {
        return new UserId(Optional.ofNullable((String) userInfoPayload.get("email"))
                .orElseThrow(() -> new ForbiddenAccessException("email is missing from headers")));
    }

    private List<Role> getRoleList(Map<String, Object> userInfoPayload) {
        return Optional.ofNullable((List<String>) userInfoPayload.get("role"))
                .orElseThrow(() -> new ForbiddenAccessException("role is missing from headers"))
                .stream().map(r -> new Role(r)).collect(Collectors.toList());
    }

    private Map<String, Object> getUserInfoMap(Map<String, String> headers) {
        String jwtPayload = getJwtPayload(headers.get("authorization"));
        return getDecodedJwtPayload(jwtPayload);
    }

    private String getJwtPayload(String bearerToken) {
        if (bearerToken == null) {
            throw new ForbiddenAccessException("No JWT Token is informed in userinfo header");
        }
        String[] chunks = bearerToken.split("[ .]+");
        return new String(Base64.getDecoder().decode(chunks[2]));
    }

    private Map<String, Object> getDecodedJwtPayload(String jwt) {
        Map<String, Object> userInfoPayload;
        try {
            userInfoPayload = new ObjectMapper().readValue(jwt, Map.class);
        } catch (JsonProcessingException e) {
            throw new RuntimeException("JWT Token informed is not compatible with ALMUserInfoAccessor impl");
        }
        return userInfoPayload;
    }



}
