unit ConstraintParticle.Simulation.BasicDistance;

interface

uses
  System.Types, System.UITypes,
  ConstraintParticle;

type
  TBasicDistanceSimulation = class(TConstraintParticleSimulation)
  public
    constructor Create(aMainRadius, aBallRadius: Single; const aStartPos: TPointF;
      aMainColor, aBallColor: TAlphaColor);
    procedure Update(const aMousePos: TPointF); override;
  end;

implementation

{ TBasicDistanceSimulation }

constructor TBasicDistanceSimulation.Create(aMainRadius, aBallRadius: Single;
  const aStartPos: TPointF; aMainColor, aBallColor: TAlphaColor);
begin
  inherited Create;

  FMainParticle.Pos := aStartPos;
  FMainParticle.Radius := aMainRadius;
  FMainParticle.Color := aMainColor;

  SetLength(FParticles, 1);
  FParticles[0].Pos := aStartPos;
  FParticles[0].Radius := aBallRadius;
  FParticles[0].Color := aBallColor;
end;

procedure TBasicDistanceSimulation.Update(const aMousePos: TPointF);
var
  toNext: TPointF;
begin
  FMainParticle.Pos := aMousePos;

  toNext := aMousePos - FParticles[0].Pos;
  if toNext.Length > FMainParticle.Radius - FParticles[0].Radius then
    FParticles[0].Pos := TConstraintResolver.Distance(
      FParticles[0].Pos, aMousePos, FMainParticle.Radius - FParticles[0].Radius);
end;

end.
