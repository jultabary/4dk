package org.tby.fourdk.security.sample.spring.infrastructure.http;

import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.tby.fourdk.security.ForbiddenAccessException;

@RestControllerAdvice
public class RestExceptionHandler {

    @ResponseBody
    @ExceptionHandler(value = {ForbiddenAccessException.class})
    @ResponseStatus(HttpStatus.FORBIDDEN)
    public String forbiddenExceptionHandler(Exception exception) {
        return "Forbidden";
    }

}
