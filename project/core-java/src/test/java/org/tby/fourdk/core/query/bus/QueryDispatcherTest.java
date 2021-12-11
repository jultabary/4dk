package org.tby.fourdk.core.query.bus;

import org.junit.jupiter.api.Test;
import test.fake.query.*;

import java.util.Arrays;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.catchThrowable;

class QueryDispatcherTest {

    @Test
    public void it_should_dispatch_a_query_to_the_correct_queryhandler() {
        // Given
        var aQuery = new AQuery();
        var aQueryHandler = new AQueryHandler();
        var anotherQueryHandler = new AnotherQueryHandler();
        var queryDispatcher = new QueryDispatcher(Arrays.asList(aQueryHandler, anotherQueryHandler));

        // When
        queryDispatcher.dispatch(aQuery);

        // Then
        assertThat(aQueryHandler.hasHandled(aQuery)).isTrue();
        assertThat(anotherQueryHandler.hasHandled(aQuery)).isFalse();


    }

    @Test
    public void it_should_throw_an_exception_if_the_query_dispatched_has_no_associated_handler() {
        // Given
        var aQuery = new AQuery();
        var anotherQueryHandler = new AnotherQueryHandler();
        var queryDispatcher = new QueryDispatcher(Arrays.asList(anotherQueryHandler));

        // When
        var raisedException = catchThrowable(() -> queryDispatcher.dispatch(aQuery));

        // Then
        assertThat(raisedException).isNotNull().isInstanceOf(UnhandledQueryException.class);
    }

    @Test
    public void it_should_only_register_one_handler_for_a_given_query() {
        // Given
        var aQueryHandlerWhichHandlesAQuery = new AQueryHandler();
        var anotherQueryHandlerWhichHandlesAQuery = new AThirdQueryHandler(new AResponse());

        // When
        var raisedException = catchThrowable(() -> new QueryDispatcher(Arrays.asList(aQueryHandlerWhichHandlesAQuery, anotherQueryHandlerWhichHandlesAQuery)));

        // Then
        assertThat(raisedException).isNotNull().isInstanceOf(QueryAlreadyHandledException.class);
    }

    @Test
    public void it_should_return_queryresponse_from_the_correct_query_handler() {
        // Given
        var aQueryResponse = new AResponse();
        var aQuery = new AQuery();
        var anotherQueryHandler = new AnotherQueryHandler();
        var aQueryHandler  = new AThirdQueryHandler(aQueryResponse);
        var queryDispatcher = new QueryDispatcher(Arrays.asList(anotherQueryHandler, aQueryHandler));

        // When
        var queryResponse = queryDispatcher.dispatch(aQuery);

        // Then
        assertThat(queryResponse).isEqualTo(aQueryResponse);
    }

}