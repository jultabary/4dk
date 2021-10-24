import org.junit.jupiter.api.Test;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.ResultActions;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;
import org.tby.fourdk.core.sample.spring.infrastructure.persistence.ParkingInMemoryRepository;

import javax.annotation.Resource;

import static org.springframework.test.web.servlet.result.MockMvcResultHandlers.print;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;
import static test.referential.TestReferential.*;

@AutoConfigureMockMvc
@EnableWebMvc
@SpringBootTest(classes = {
        AppConfig.class
})
class ApplicationE2ETest {

    @Resource
    private MockMvc mockMvc;

    @Resource
    private ParkingInMemoryRepository parkingRepository;

    @Test
    void it_should_dispatch_ParkACarCommand_and_succeed() throws Exception {
        // Given
        this.parkingRepository.add(A_PARKING);
        String request = "{" +
                "\"carId\": \"" + A_CAR.id() + "\"," +
                "\"parkingId\": \"" + A_PARKING.getId().id() + "\"" +
                "}";
        var httpRequest = MockMvcRequestBuilders.post("/parkACar")
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)
                .content(request);

        // When
        ResultActions performGet = this.mockMvc.perform(httpRequest)
                .andDo(print());

        // Then
        performGet.andExpect(status().isOk());
    }

    @Test
    void it_should_dispatch_ParkACarCommand_and_failed() throws Exception {
        // Given
        this.parkingRepository.add(A_SMALL_PARKING);
        A_SMALL_PARKING.parkACar(ANOTHER_CAR);
        String request = "{" +
                "\"carId\": \"" + A_CAR.id() + "\"," +
                "\"parkingId\": \"" + A_SMALL_PARKING.getId().id() + "\"" +
                "}";
        var httpRequest = MockMvcRequestBuilders.post("/parkACar")
                .contentType(MediaType.APPLICATION_JSON)
                .accept(MediaType.APPLICATION_JSON)
                .content(request);

        // When
        ResultActions performGet = this.mockMvc.perform(httpRequest)
                .andDo(print());

        // Then
        performGet.andExpect(status().isConflict());
    }

}