import org.springframework.boot.autoconfigure.EnableAutoConfiguration;
import org.springframework.context.annotation.ComponentScan;
import org.springframework.context.annotation.Configuration;

@Configuration
@ComponentScan(
        basePackages = {
                "org.tby.fourdk.security"
        }
)
@EnableAutoConfiguration
public class AppConfig {
}
