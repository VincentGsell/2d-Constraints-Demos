unit constraintSimple.fmain;

interface

uses
  System.SysUtils, System.Types, System.UITypes, System.Classes, System.Variants,
  FMX.Types, FMX.Controls, FMX.Forms, FMX.Graphics, FMX.Dialogs, FMX.Objects,
  FMX.Controls.Presentation, FMX.StdCtrls,
  ConstraintParticle,
  ConstraintParticle.Renderer.FMX2D,
  ConstraintParticle.Simulation.BasicDistance,
  ConstraintParticle.Simulation.SeparateCollision,
  ConstraintParticle.Simulation.DistanceChain;

type
  TConstraintScene = (BasicDistance, SeparateCollision, DistanceChain);

  TFMain = class(TForm)
    Selection1: TSelection;
    Rectangle1: TRectangle;
    CornerButton1: TCornerButton;
    CornerButton2: TCornerButton;
    CornerButton3: TCornerButton;
    SelectionConstraintChainSubMenu: TSelection;
    Rectangle2: TRectangle;
    CheckBoxFabrick: TCheckBox;
    TrackBar1: TTrackBar;
    cbBallCollision: TCheckBox;
    procedure FormCreate(Sender: TObject);
    procedure FormDestroy(Sender: TObject);
    procedure CornerButton1Click(Sender: TObject);
    procedure FormMouseMove(Sender: TObject; Shift: TShiftState; X, Y: Single);
    procedure CheckBoxFabrickChange(Sender: TObject);
    procedure cbBallCollisionChange(Sender: TObject);
    procedure TrackBar1Change(Sender: TObject);
  private
    FCurrentScene: TConstraintScene;
    FSimulation: TConstraintParticleSimulation;
    FRenderer: TConstraintParticleFMX2DRenderer;
    procedure SetScene(const Value: TConstraintScene);
  public
    property Scene: TConstraintScene read FCurrentScene write SetScene;
  end;

var
  FMain: TFMain;

const
  cst_SEPARATECOLL_BALL_COUNT = 4000;
  cst_SEPARATECOLL_BALLSIZE = 5;
  cst_SEPARATECOLL_ITERATIONS = 3;
  cst_CHAIN_BALL_COUNT = 20;
  cst_CHAIN_LINK_DISTANCE = 30;

implementation

{$R *.fmx}

procedure TFMain.FormCreate(Sender: TObject);
begin
  FRenderer := TConstraintParticleFMX2DRenderer.Create(Self);
  OnMouseMove := FormMouseMove;
  SelectionConstraintChainSubMenu.Visible := False;
  Scene := TConstraintScene.BasicDistance;
end;

procedure TFMain.FormDestroy(Sender: TObject);
begin
  FreeAndNil(FSimulation);
  FreeAndNil(FRenderer);
end;

procedure TFMain.FormMouseMove(Sender: TObject; Shift: TShiftState; X, Y: Single);
begin
  if FSimulation = nil then
    Exit;

  //Update chain options before update
  if FSimulation is TDistanceChainSimulation then
  begin
    TDistanceChainSimulation(FSimulation).AnchorPos := PointF(ClientWidth / 2, ClientHeight / 2);
  end;

  FSimulation.Update(PointF(X, Y));
  FRenderer.Render(FSimulation);
end;

procedure TFMain.CornerButton1Click(Sender: TObject);
begin
  case TCornerButton(Sender).Tag of
    10: Scene := TConstraintScene.BasicDistance;
    20: Scene := TConstraintScene.SeparateCollision;
    30: Scene := TConstraintScene.DistanceChain;
  end;
end;

procedure TFMain.CheckBoxFabrickChange(Sender: TObject);
begin
  if FSimulation is TDistanceChainSimulation then
    TDistanceChainSimulation(FSimulation).UseFabrik := CheckBoxFabrick.IsChecked;
end;

procedure TFMain.cbBallCollisionChange(Sender: TObject);
begin
  if FSimulation is TDistanceChainSimulation then
    TDistanceChainSimulation(FSimulation).BallCollision := cbBallCollision.IsChecked;
end;

procedure TFMain.TrackBar1Change(Sender: TObject);
begin
  if FSimulation is TDistanceChainSimulation then
    TDistanceChainSimulation(FSimulation).LinkDistance := TrackBar1.Value;
end;

procedure TFMain.SetScene(const Value: TConstraintScene);
begin
  FCurrentScene := Value;

  CornerButton1.IsPressed := False;
  CornerButton2.IsPressed := False;
  CornerButton3.IsPressed := False;
  SelectionConstraintChainSubMenu.Visible := False;

  //Clear renderer reference before freeing simulation
  FRenderer.Render(nil);
  FreeAndNil(FSimulation);

  case Value of
    TConstraintScene.BasicDistance:
    begin
      CornerButton1.IsPressed := True;
      FSimulation := TBasicDistanceSimulation.Create(
        50, 15,
        PointF(400, 400),
        TAlphaColors.White, TAlphaColors.Black);
    end;

    TConstraintScene.SeparateCollision:
    begin
      CornerButton2.IsPressed := True;
      FSimulation := TSeparateCollisionSimulation.Create(
        cst_SEPARATECOLL_BALL_COUNT,
        cst_SEPARATECOLL_BALLSIZE,
        50,
        PointF(400, 400),
        TAlphaColors.White,
        True);
      TSeparateCollisionSimulation(FSimulation).Iterations := cst_SEPARATECOLL_ITERATIONS;
    end;

    TConstraintScene.DistanceChain:
    begin
      CornerButton3.IsPressed := True;
      SelectionConstraintChainSubMenu.Visible := True;
      FSimulation := TDistanceChainSimulation.Create(
        cst_CHAIN_BALL_COUNT,
        15,
        cst_CHAIN_LINK_DISTANCE,
        PointF(400, 400),
        True);
      TDistanceChainSimulation(FSimulation).UseFabrik := CheckBoxFabrick.IsChecked;
      TDistanceChainSimulation(FSimulation).BallCollision := cbBallCollision.IsChecked;
      TDistanceChainSimulation(FSimulation).LinkDistance := TrackBar1.Value;
      FRenderer.RenderMainParticle := False;
    end;
  end;

  //Reset render options
  if Value <> TConstraintScene.DistanceChain then
    FRenderer.RenderMainParticle := True;

  FRenderer.PaintBox.BringToFront;
  Selection1.BringToFront;
end;

end.
