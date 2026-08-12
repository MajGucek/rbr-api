#include "IPlugin.h"

extern "C"
{
    void* rust_plugin_create(void* game);
    void rust_plugin_destroy(void* state);

    const char* rust_plugin_get_name(void* state);

    void rust_plugin_draw_frontend_page(void* state);
    void rust_plugin_draw_results_ui(void* state);

    void rust_plugin_handle_frontend_events(
        void* state,
        char keyboard,
        unsigned char up,
        unsigned char down,
        unsigned char left,
        unsigned char right,
        unsigned char select
    );

    void rust_plugin_tick_frontend_page(
        void* state,
        float delta
    );

    void rust_plugin_stage_started(
        void* state,
        int map,
        const char* player_name,
        unsigned char false_start
    );

    void rust_plugin_handle_results(
        void* state,
        float checkpoint1,
        float checkpoint2,
        float finish_time,
        const char* player_name
    );

    void rust_plugin_checkpoint(
        void* state,
        float checkpoint_time,
        int checkpoint_id,
        const char* player_name
    );
}

class RustPlugin final : public IPlugin
{
private:
    void* state;

public:
    explicit RustPlugin(void* game)
        : state(rust_plugin_create(game))
    {
    }

    ~RustPlugin() override
    {
        rust_plugin_destroy(state);
    }

    const char* GetName() override
    {
        return rust_plugin_get_name(state);
    }

    void DrawFrontEndPage() override
    {
        rust_plugin_draw_frontend_page(state);
    }

    void DrawResultsUI() override
    {
        rust_plugin_draw_results_ui(state);
    }

    void HandleFrontEndEvents(
        char keyboard,
        bool up,
        bool down,
        bool left,
        bool right,
        bool select
    ) override
    {
        rust_plugin_handle_frontend_events(
            state,
            keyboard,
            up,
            down,
            left,
            right,
            select
        );
    }

    void TickFrontEndPage(float delta) override
    {
        rust_plugin_tick_frontend_page(state, delta);
    }

    void StageStarted(
        int map,
        const char* player_name,
        bool false_start
    ) override
    {
        rust_plugin_stage_started(
            state,
            map,
            player_name,
            false_start
        );
    }

    void HandleResults(
        float checkpoint1,
        float checkpoint2,
        float finish_time,
        const char* player_name
    ) override
    {
        rust_plugin_handle_results(
            state,
            checkpoint1,
            checkpoint2,
            finish_time,
            player_name
        );
    }

    void CheckPoint(
        float checkpoint_time,
        int checkpoint_id,
        const char* player_name
    ) override
    {
        rust_plugin_checkpoint(
            state,
            checkpoint_time,
            checkpoint_id,
            player_name
        );
    }
};

extern "C" IPlugin* cpp_create_plugin(void* game)
{
    return new RustPlugin(game);
}