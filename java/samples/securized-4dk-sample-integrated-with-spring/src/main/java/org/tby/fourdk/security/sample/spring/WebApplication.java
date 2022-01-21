package org.tby.fourdk.security.sample.spring;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.ComponentScan;

@ComponentScan("org.tby")
@SpringBootApplication
public class WebApplication {

        public static void main(String[] args) {
            SpringApplication.run(WebApplication.class, args);
        }

}
