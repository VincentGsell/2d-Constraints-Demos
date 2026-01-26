unit ConstraintParticle.Renderer.FMX2D;

interface

uses
  System.Types, System.UITypes,
  FMX.Types, FMX.Graphics, FMX.Objects,
  ConstraintParticle;

type
  TConstraintParticleFMX2DRenderer = class
  private
    FPaintBox: TPaintBox;
    FSimulation: TConstraintParticleSimulation;
    FRenderMainParticle: Boolean;
    procedure DoPaint(Sender: TObject; Canvas: TCanvas);
  public
    constructor Create(aParent: TFmxObject);
    destructor Destroy; override;
    procedure Render(const aSimulation: TConstraintParticleSimulation);

    property RenderMainParticle: Boolean read FRenderMainParticle write FRenderMainParticle;
    property PaintBox: TPaintBox read FPaintBox;
  end;

implementation

uses
  FMX.Controls;

{ TConstraintParticleFMX2DRenderer }

constructor TConstraintParticleFMX2DRenderer.Create(aParent: TFmxObject);
begin
  inherited Create;
  FRenderMainParticle := True;

  FPaintBox := TPaintBox.Create(nil);
  FPaintBox.Parent := aParent;
  FPaintBox.Align := TAlignLayout.Client;
  FPaintBox.OnPaint := DoPaint;
  FPaintBox.HitTest := False;
end;

destructor TConstraintParticleFMX2DRenderer.Destroy;
begin
  FPaintBox.Free;
  inherited;
end;

procedure TConstraintParticleFMX2DRenderer.DoPaint(Sender: TObject; Canvas: TCanvas);
var
  i: Integer;
  r: TRectF;
  p: TConstraintParticle;
begin
  if FSimulation = nil then
    Exit;

  //Main particle
  if FRenderMainParticle then
  begin
    p := FSimulation.MainParticle;
    Canvas.Fill.Color := p.Color;
    r := RectF(
      p.Pos.X - p.Radius,
      p.Pos.Y - p.Radius,
      p.Pos.X + p.Radius,
      p.Pos.Y + p.Radius
    );
    Canvas.FillEllipse(r, 1);
  end;

  //All particles
  for i := 0 to FSimulation.ParticleCount - 1 do
  begin
    p := FSimulation.Particles[i];
    Canvas.Fill.Color := p.Color;
    r := RectF(
      p.Pos.X - p.Radius,
      p.Pos.Y - p.Radius,
      p.Pos.X + p.Radius,
      p.Pos.Y + p.Radius
    );
    Canvas.FillEllipse(r, 1);
  end;
end;

procedure TConstraintParticleFMX2DRenderer.Render(const aSimulation: TConstraintParticleSimulation);
begin
  FSimulation := aSimulation;
  FPaintBox.Repaint;
end;

end.
